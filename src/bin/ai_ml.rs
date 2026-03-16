// Rust Tutorial #28: Rust for AI/ML — Concepts and Patterns
// Demonstrates data processing, matrix operations, linear algebra basics,
// and AI/ML patterns using standard Rust (no heavy dependencies).

use std::collections::HashMap;

// ========================================================
// Concept 1: Ndarray-like Matrix Operations
// ========================================================

#[derive(Debug, Clone, PartialEq)]
struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    fn from_vec(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), rows * cols, "Data length must match rows * cols");
        Self { data, rows, cols }
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                result.set(c, r, self.get(r, c));
            }
        }
        result
    }

    fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows, "Matrix dimensions must match");
        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        result
    }

    fn add(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.shape(), other.shape(), "Matrix shapes must match");
        let data: Vec<f64> = self.data.iter().zip(&other.data).map(|(a, b)| a + b).collect();
        Matrix::from_vec(self.rows, self.cols, data)
    }

    fn scalar_multiply(&self, scalar: f64) -> Matrix {
        let data: Vec<f64> = self.data.iter().map(|v| v * scalar).collect();
        Matrix::from_vec(self.rows, self.cols, data)
    }

    fn sum(&self) -> f64 {
        self.data.iter().sum()
    }

    fn mean(&self) -> f64 {
        self.sum() / self.data.len() as f64
    }

    fn apply(&self, f: impl Fn(f64) -> f64) -> Matrix {
        let data: Vec<f64> = self.data.iter().map(|v| f(*v)).collect();
        Matrix::from_vec(self.rows, self.cols, data)
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows {
            let row: Vec<String> = (0..self.cols).map(|c| format!("{:.2}", self.get(r, c))).collect();
            writeln!(f, "[{}]", row.join(", "))?;
        }
        Ok(())
    }
}

// ========================================================
// Concept 2: DataFrame-like Operations
// ========================================================

#[derive(Debug, Clone)]
enum ColumnData {
    Float(Vec<f64>),
    Int(Vec<i64>),
    Text(Vec<String>),
}

impl ColumnData {
    fn len(&self) -> usize {
        match self {
            ColumnData::Float(v) => v.len(),
            ColumnData::Int(v) => v.len(),
            ColumnData::Text(v) => v.len(),
        }
    }
}

struct DataFrame {
    columns: Vec<(String, ColumnData)>,
}

impl DataFrame {
    fn new() -> Self {
        Self { columns: Vec::new() }
    }

    fn add_column(&mut self, name: &str, data: ColumnData) {
        self.columns.push((name.to_string(), data));
    }

    fn num_rows(&self) -> usize {
        self.columns.first().map(|(_, d)| d.len()).unwrap_or(0)
    }

    fn num_cols(&self) -> usize {
        self.columns.len()
    }

    fn column_names(&self) -> Vec<&str> {
        self.columns.iter().map(|(name, _)| name.as_str()).collect()
    }

    fn get_float_column(&self, name: &str) -> Option<&Vec<f64>> {
        for (col_name, data) in &self.columns {
            if col_name == name {
                if let ColumnData::Float(v) = data {
                    return Some(v);
                }
            }
        }
        None
    }

    fn get_int_column(&self, name: &str) -> Option<&Vec<i64>> {
        for (col_name, data) in &self.columns {
            if col_name == name {
                if let ColumnData::Int(v) = data {
                    return Some(v);
                }
            }
        }
        None
    }

    fn describe_float(&self, name: &str) -> Option<ColumnStats> {
        let col = self.get_float_column(name)?;
        if col.is_empty() {
            return None;
        }
        let sum: f64 = col.iter().sum();
        let mean = sum / col.len() as f64;
        let min = col.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = col.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let variance = col.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / col.len() as f64;
        let std_dev = variance.sqrt();

        Some(ColumnStats { count: col.len(), mean, min, max, std_dev })
    }

    fn filter_by_float(&self, column: &str, predicate: impl Fn(f64) -> bool) -> Vec<usize> {
        if let Some(col) = self.get_float_column(column) {
            col.iter().enumerate()
                .filter(|(_, v)| predicate(**v))
                .map(|(i, _)| i)
                .collect()
        } else {
            vec![]
        }
    }

    fn group_by_int_sum_float(&self, group_col: &str, sum_col: &str) -> HashMap<i64, f64> {
        let groups = self.get_int_column(group_col);
        let values = self.get_float_column(sum_col);

        match (groups, values) {
            (Some(g), Some(v)) => {
                let mut result: HashMap<i64, f64> = HashMap::new();
                for (group, value) in g.iter().zip(v.iter()) {
                    *result.entry(*group).or_insert(0.0) += value;
                }
                result
            }
            _ => HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct ColumnStats {
    count: usize,
    mean: f64,
    min: f64,
    max: f64,
    std_dev: f64,
}

impl std::fmt::Display for ColumnStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "count: {}, mean: {:.2}, min: {:.2}, max: {:.2}, std: {:.2}",
            self.count, self.mean, self.min, self.max, self.std_dev
        )
    }
}

// ========================================================
// Concept 3: Simple Neural Network (Single Neuron)
// ========================================================

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn sigmoid_derivative(x: f64) -> f64 {
    let s = sigmoid(x);
    s * (1.0 - s)
}

fn relu(x: f64) -> f64 {
    if x > 0.0 { x } else { 0.0 }
}

fn relu_derivative(x: f64) -> f64 {
    if x > 0.0 { 1.0 } else { 0.0 }
}

struct Neuron {
    weights: Vec<f64>,
    bias: f64,
    learning_rate: f64,
}

impl Neuron {
    fn new(num_inputs: usize, learning_rate: f64) -> Self {
        // Simple initialization
        let weights = vec![0.5; num_inputs];
        Self {
            weights,
            bias: 0.0,
            learning_rate,
        }
    }

    fn forward(&self, inputs: &[f64]) -> f64 {
        let sum: f64 = self.weights.iter().zip(inputs).map(|(w, x)| w * x).sum::<f64>() + self.bias;
        sigmoid(sum)
    }

    fn train(&mut self, inputs: &[f64], target: f64) -> f64 {
        let prediction = self.forward(inputs);
        let error = target - prediction;
        let gradient = error * sigmoid_derivative(
            self.weights.iter().zip(inputs).map(|(w, x)| w * x).sum::<f64>() + self.bias
        );

        for (w, x) in self.weights.iter_mut().zip(inputs) {
            *w += self.learning_rate * gradient * x;
        }
        self.bias += self.learning_rate * gradient;

        error.powi(2)
    }
}

// ========================================================
// Concept 4: K-Nearest Neighbors
// ========================================================

#[derive(Debug, Clone)]
struct DataPoint {
    features: Vec<f64>,
    label: String,
}

fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt()
}

fn knn_classify(train_data: &[DataPoint], query: &[f64], k: usize) -> String {
    let mut distances: Vec<(f64, &str)> = train_data
        .iter()
        .map(|point| (euclidean_distance(&point.features, query), point.label.as_str()))
        .collect();

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut votes: HashMap<&str, usize> = HashMap::new();
    for (_, label) in distances.iter().take(k) {
        *votes.entry(label).or_insert(0) += 1;
    }

    votes.into_iter().max_by_key(|(_, count)| *count).map(|(label, _)| label.to_string()).unwrap_or_default()
}

// ========================================================
// Concept 5: Linear Regression
// ========================================================

fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y).map(|(a, b)| a * b).sum();
    let sum_x2: f64 = x.iter().map(|a| a * a).sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / n;

    (slope, intercept)
}

fn predict_linear(slope: f64, intercept: f64, x: f64) -> f64 {
    slope * x + intercept
}

fn r_squared(x: &[f64], y: &[f64], slope: f64, intercept: f64) -> f64 {
    let mean_y = y.iter().sum::<f64>() / y.len() as f64;
    let ss_tot: f64 = y.iter().map(|yi| (yi - mean_y).powi(2)).sum();
    let ss_res: f64 = x.iter().zip(y).map(|(xi, yi)| {
        let pred = predict_linear(slope, intercept, *xi);
        (yi - pred).powi(2)
    }).sum();

    1.0 - ss_res / ss_tot
}

// ========================================================
// Concept 6: Normalization and Feature Scaling
// ========================================================

fn min_max_normalize(data: &[f64]) -> Vec<f64> {
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;
    if range == 0.0 {
        return vec![0.0; data.len()];
    }
    data.iter().map(|v| (v - min) / range).collect()
}

fn z_score_normalize(data: &[f64]) -> Vec<f64> {
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let variance = data.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / data.len() as f64;
    let std_dev = variance.sqrt();
    if std_dev == 0.0 {
        return vec![0.0; data.len()];
    }
    data.iter().map(|v| (v - mean) / std_dev).collect()
}

// ========================================================
// Concept 7: Confusion Matrix
// ========================================================

struct ConfusionMatrix {
    true_positive: usize,
    true_negative: usize,
    false_positive: usize,
    false_negative: usize,
}

impl ConfusionMatrix {
    fn from_predictions(actual: &[bool], predicted: &[bool]) -> Self {
        let mut cm = Self {
            true_positive: 0,
            true_negative: 0,
            false_positive: 0,
            false_negative: 0,
        };

        for (a, p) in actual.iter().zip(predicted) {
            match (a, p) {
                (true, true) => cm.true_positive += 1,
                (true, false) => cm.false_negative += 1,
                (false, true) => cm.false_positive += 1,
                (false, false) => cm.true_negative += 1,
            }
        }

        cm
    }

    fn accuracy(&self) -> f64 {
        let total = self.true_positive + self.true_negative + self.false_positive + self.false_negative;
        if total == 0 { return 0.0; }
        (self.true_positive + self.true_negative) as f64 / total as f64
    }

    fn precision(&self) -> f64 {
        let denom = self.true_positive + self.false_positive;
        if denom == 0 { return 0.0; }
        self.true_positive as f64 / denom as f64
    }

    fn recall(&self) -> f64 {
        let denom = self.true_positive + self.false_negative;
        if denom == 0 { return 0.0; }
        self.true_positive as f64 / denom as f64
    }

    fn f1_score(&self) -> f64 {
        let p = self.precision();
        let r = self.recall();
        if p + r == 0.0 { return 0.0; }
        2.0 * p * r / (p + r)
    }
}

fn main() {
    println!("=== Rust for AI/ML Demo ===\n");

    // Demo 1: Matrix operations
    println!("--- Matrix Operations ---");
    let a = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let b = Matrix::from_vec(3, 2, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
    println!("A (2x3):\n{}", a);
    println!("B (3x2):\n{}", b);
    let product = a.multiply(&b);
    println!("A * B (2x2):\n{}", product);

    let t = a.transpose();
    println!("A^T (3x2):\n{}", t);

    println!();

    // Demo 2: DataFrame
    println!("--- DataFrame ---");
    let mut df = DataFrame::new();
    df.add_column("age", ColumnData::Float(vec![25.0, 30.0, 35.0, 28.0, 42.0]));
    df.add_column("salary", ColumnData::Float(vec![50000.0, 65000.0, 80000.0, 55000.0, 95000.0]));
    df.add_column("department", ColumnData::Int(vec![1, 2, 1, 2, 1]));

    println!("Shape: {} rows x {} cols", df.num_rows(), df.num_cols());
    println!("Columns: {:?}", df.column_names());

    if let Some(stats) = df.describe_float("salary") {
        println!("Salary stats: {}", stats);
    }

    let high_salary = df.filter_by_float("salary", |v| v > 60000.0);
    println!("High salary indices: {:?}", high_salary);

    let dept_totals = df.group_by_int_sum_float("department", "salary");
    println!("Salary by department: {:?}", dept_totals);

    println!();

    // Demo 3: Neural network (single neuron learning AND gate)
    println!("--- Single Neuron (AND gate) ---");
    let mut neuron = Neuron::new(2, 0.5);

    let training_data = vec![
        (vec![0.0, 0.0], 0.0),
        (vec![0.0, 1.0], 0.0),
        (vec![1.0, 0.0], 0.0),
        (vec![1.0, 1.0], 1.0),
    ];

    for epoch in 0..1000 {
        let mut total_error = 0.0;
        for (inputs, target) in &training_data {
            total_error += neuron.train(inputs, *target);
        }
        if epoch % 200 == 0 {
            println!("  Epoch {}: error = {:.4}", epoch, total_error);
        }
    }

    println!("Predictions after training:");
    for (inputs, _) in &training_data {
        let pred = neuron.forward(inputs);
        println!("  {:?} -> {:.3} (rounded: {})", inputs, pred, if pred > 0.5 { 1 } else { 0 });
    }

    println!();

    // Demo 4: KNN
    println!("--- K-Nearest Neighbors ---");
    let train_data = vec![
        DataPoint { features: vec![1.0, 1.0], label: "A".to_string() },
        DataPoint { features: vec![1.5, 2.0], label: "A".to_string() },
        DataPoint { features: vec![3.0, 3.0], label: "B".to_string() },
        DataPoint { features: vec![5.0, 4.0], label: "B".to_string() },
        DataPoint { features: vec![3.5, 3.5], label: "B".to_string() },
    ];

    let query = vec![2.0, 2.0];
    let label = knn_classify(&train_data, &query, 3);
    println!("Query {:?} classified as: {}", query, label);

    println!();

    // Demo 5: Linear regression
    println!("--- Linear Regression ---");
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.1, 3.9, 6.2, 7.8, 10.1];
    let (slope, intercept) = linear_regression(&x, &y);
    println!("y = {:.2}x + {:.2}", slope, intercept);
    let r2 = r_squared(&x, &y, slope, intercept);
    println!("R-squared: {:.4}", r2);
    println!("Predict x=6: {:.2}", predict_linear(slope, intercept, 6.0));

    println!();

    // Demo 6: Normalization
    println!("--- Feature Scaling ---");
    let data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    let min_max = min_max_normalize(&data);
    let z_score = z_score_normalize(&data);
    println!("Original: {:?}", data);
    println!("Min-Max: {:?}", min_max);
    println!("Z-Score: {:?}", z_score.iter().map(|v| format!("{:.2}", v)).collect::<Vec<_>>());

    println!();

    // Demo 7: Confusion matrix
    println!("--- Confusion Matrix ---");
    let actual = vec![true, true, false, true, false, true, false, false, true, true];
    let predicted = vec![true, true, false, false, false, true, true, false, true, false];
    let cm = ConfusionMatrix::from_predictions(&actual, &predicted);
    println!("TP: {}, TN: {}, FP: {}, FN: {}", cm.true_positive, cm.true_negative, cm.false_positive, cm.false_negative);
    println!("Accuracy: {:.2}", cm.accuracy());
    println!("Precision: {:.2}", cm.precision());
    println!("Recall: {:.2}", cm.recall());
    println!("F1 Score: {:.2}", cm.f1_score());

    println!();

    // Demo 8: Activation functions
    println!("--- Activation Functions ---");
    for x in [-2.0, -1.0, 0.0, 1.0, 2.0] {
        println!("  x={:.1}: sigmoid={:.3}, relu={:.1}", x, sigmoid(x), relu(x));
    }

    println!("\n=== All demos completed! ===");
}

// ========================================================
// Tests
// ========================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Matrix ---

    #[test]
    fn test_matrix_create_and_access() {
        let mut m = Matrix::new(2, 3);
        m.set(0, 0, 1.0);
        m.set(1, 2, 5.0);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 2), 5.0);
        assert_eq!(m.get(0, 1), 0.0);
    }

    #[test]
    fn test_matrix_shape() {
        let m = Matrix::new(3, 4);
        assert_eq!(m.shape(), (3, 4));
    }

    #[test]
    fn test_matrix_transpose() {
        let m = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let t = m.transpose();
        assert_eq!(t.shape(), (3, 2));
        assert_eq!(t.get(0, 0), 1.0);
        assert_eq!(t.get(0, 1), 4.0);
        assert_eq!(t.get(2, 0), 3.0);
    }

    #[test]
    fn test_matrix_multiply() {
        let a = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let b = Matrix::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let c = a.multiply(&b);
        assert_eq!(c.get(0, 0), 19.0); // 1*5 + 2*7
        assert_eq!(c.get(0, 1), 22.0); // 1*6 + 2*8
        assert_eq!(c.get(1, 0), 43.0); // 3*5 + 4*7
        assert_eq!(c.get(1, 1), 50.0); // 3*6 + 4*8
    }

    #[test]
    fn test_matrix_add() {
        let a = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let b = Matrix::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let c = a.add(&b);
        assert_eq!(c.data, vec![6.0, 8.0, 10.0, 12.0]);
    }

    #[test]
    fn test_matrix_scalar_multiply() {
        let m = Matrix::from_vec(1, 3, vec![1.0, 2.0, 3.0]);
        let scaled = m.scalar_multiply(2.0);
        assert_eq!(scaled.data, vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_matrix_sum_mean() {
        let m = Matrix::from_vec(1, 4, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m.sum(), 10.0);
        assert_eq!(m.mean(), 2.5);
    }

    #[test]
    fn test_matrix_apply() {
        let m = Matrix::from_vec(1, 3, vec![1.0, 4.0, 9.0]);
        let result = m.apply(|x| x.sqrt());
        assert_eq!(result.data, vec![1.0, 2.0, 3.0]);
    }

    // --- DataFrame ---

    #[test]
    fn test_dataframe_shape() {
        let mut df = DataFrame::new();
        df.add_column("a", ColumnData::Float(vec![1.0, 2.0, 3.0]));
        df.add_column("b", ColumnData::Int(vec![4, 5, 6]));
        assert_eq!(df.num_rows(), 3);
        assert_eq!(df.num_cols(), 2);
    }

    #[test]
    fn test_dataframe_describe() {
        let mut df = DataFrame::new();
        df.add_column("vals", ColumnData::Float(vec![10.0, 20.0, 30.0]));
        let stats = df.describe_float("vals").unwrap();
        assert_eq!(stats.count, 3);
        assert!((stats.mean - 20.0).abs() < 0.01);
        assert!((stats.min - 10.0).abs() < 0.01);
        assert!((stats.max - 30.0).abs() < 0.01);
    }

    #[test]
    fn test_dataframe_filter() {
        let mut df = DataFrame::new();
        df.add_column("vals", ColumnData::Float(vec![1.0, 5.0, 3.0, 7.0]));
        let indices = df.filter_by_float("vals", |v| v > 4.0);
        assert_eq!(indices, vec![1, 3]);
    }

    #[test]
    fn test_dataframe_group_by() {
        let mut df = DataFrame::new();
        df.add_column("group", ColumnData::Int(vec![1, 2, 1, 2]));
        df.add_column("value", ColumnData::Float(vec![10.0, 20.0, 30.0, 40.0]));
        let groups = df.group_by_int_sum_float("group", "value");
        assert_eq!(groups[&1], 40.0);
        assert_eq!(groups[&2], 60.0);
    }

    // --- Activation Functions ---

    #[test]
    fn test_sigmoid() {
        assert!((sigmoid(0.0) - 0.5).abs() < 0.001);
        assert!(sigmoid(10.0) > 0.99);
        assert!(sigmoid(-10.0) < 0.01);
    }

    #[test]
    fn test_sigmoid_derivative() {
        // At x=0, sigmoid'(0) = 0.25
        assert!((sigmoid_derivative(0.0) - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_relu() {
        assert_eq!(relu(-5.0), 0.0);
        assert_eq!(relu(0.0), 0.0);
        assert_eq!(relu(3.0), 3.0);
    }

    #[test]
    fn test_relu_derivative() {
        assert_eq!(relu_derivative(-1.0), 0.0);
        assert_eq!(relu_derivative(1.0), 1.0);
    }

    // --- Neuron ---

    #[test]
    fn test_neuron_forward() {
        let neuron = Neuron::new(2, 0.1);
        let output = neuron.forward(&[1.0, 1.0]);
        assert!(output > 0.0 && output < 1.0); // sigmoid output
    }

    #[test]
    fn test_neuron_learns_and_gate() {
        let mut neuron = Neuron::new(2, 0.5);
        let data = vec![
            (vec![0.0, 0.0], 0.0),
            (vec![0.0, 1.0], 0.0),
            (vec![1.0, 0.0], 0.0),
            (vec![1.0, 1.0], 1.0),
        ];

        for _ in 0..2000 {
            for (inputs, target) in &data {
                neuron.train(inputs, *target);
            }
        }

        assert!(neuron.forward(&[0.0, 0.0]) < 0.3);
        assert!(neuron.forward(&[0.0, 1.0]) < 0.3);
        assert!(neuron.forward(&[1.0, 0.0]) < 0.3);
        assert!(neuron.forward(&[1.0, 1.0]) > 0.7);
    }

    // --- KNN ---

    #[test]
    fn test_euclidean_distance() {
        assert!((euclidean_distance(&[0.0, 0.0], &[3.0, 4.0]) - 5.0).abs() < 0.001);
        assert!((euclidean_distance(&[1.0, 1.0], &[1.0, 1.0]) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_knn_classify() {
        let data = vec![
            DataPoint { features: vec![0.0, 0.0], label: "A".to_string() },
            DataPoint { features: vec![0.0, 1.0], label: "A".to_string() },
            DataPoint { features: vec![10.0, 10.0], label: "B".to_string() },
            DataPoint { features: vec![10.0, 11.0], label: "B".to_string() },
        ];
        assert_eq!(knn_classify(&data, &[0.5, 0.5], 3), "A");
        assert_eq!(knn_classify(&data, &[10.0, 10.5], 3), "B");
    }

    // --- Linear Regression ---

    #[test]
    fn test_linear_regression_perfect() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // y = 2x
        let (slope, intercept) = linear_regression(&x, &y);
        assert!((slope - 2.0).abs() < 0.001);
        assert!((intercept - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_linear_regression_with_intercept() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![3.0, 5.0, 7.0, 9.0, 11.0]; // y = 2x + 1
        let (slope, intercept) = linear_regression(&x, &y);
        assert!((slope - 2.0).abs() < 0.001);
        assert!((intercept - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_r_squared_perfect() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        let (slope, intercept) = linear_regression(&x, &y);
        let r2 = r_squared(&x, &y, slope, intercept);
        assert!((r2 - 1.0).abs() < 0.001);
    }

    // --- Normalization ---

    #[test]
    fn test_min_max_normalize() {
        let data = vec![0.0, 50.0, 100.0];
        let normalized = min_max_normalize(&data);
        assert_eq!(normalized, vec![0.0, 0.5, 1.0]);
    }

    #[test]
    fn test_min_max_normalize_same() {
        let data = vec![5.0, 5.0, 5.0];
        let normalized = min_max_normalize(&data);
        assert_eq!(normalized, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_z_score_normalize() {
        let data = vec![10.0, 20.0, 30.0];
        let normalized = z_score_normalize(&data);
        assert!((normalized[1] - 0.0).abs() < 0.001); // mean is centered
    }

    // --- Confusion Matrix ---

    #[test]
    fn test_confusion_matrix_perfect() {
        let actual = vec![true, true, false, false];
        let predicted = vec![true, true, false, false];
        let cm = ConfusionMatrix::from_predictions(&actual, &predicted);
        assert_eq!(cm.accuracy(), 1.0);
        assert_eq!(cm.precision(), 1.0);
        assert_eq!(cm.recall(), 1.0);
        assert_eq!(cm.f1_score(), 1.0);
    }

    #[test]
    fn test_confusion_matrix_half() {
        let actual = vec![true, true, false, false];
        let predicted = vec![true, false, true, false];
        let cm = ConfusionMatrix::from_predictions(&actual, &predicted);
        assert_eq!(cm.true_positive, 1);
        assert_eq!(cm.true_negative, 1);
        assert_eq!(cm.false_positive, 1);
        assert_eq!(cm.false_negative, 1);
        assert_eq!(cm.accuracy(), 0.5);
    }

    #[test]
    fn test_confusion_matrix_f1() {
        let actual = vec![true, true, true, false, false];
        let predicted = vec![true, true, false, false, false];
        let cm = ConfusionMatrix::from_predictions(&actual, &predicted);
        assert!((cm.precision() - 1.0).abs() < 0.001);
        assert!((cm.recall() - 2.0 / 3.0).abs() < 0.001);
    }
}
