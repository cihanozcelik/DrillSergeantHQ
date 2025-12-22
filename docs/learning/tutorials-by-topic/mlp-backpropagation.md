## MLPs and Backpropagation: A Standalone Guide With an Intuition-First Derivation

### Who this is for
You’ve used neural networks, but the math behind backprop still feels like magic. This tutorial explains a basic **multi-layer perceptron (MLP)** and backpropagation in a way you can implement from scratch.

### What you’ll learn
- What an MLP is (layers, weights, biases, activations)
- What “forward pass” and “loss” mean
- How backprop uses the **chain rule**
- The gradient formulas for a linear layer + activation
- A worked example showing shapes and steps

---

## 1) What is an MLP?

An **MLP** is a stack of layers that apply:

1) a linear transform \(z = Wx + b\)  
2) a nonlinearity \(a = \sigma(z)\)

Repeated several times, then a final output layer.

### Common shapes

- Input vector: \(x \in \mathbb{R}^{d}\)
- Weights: \(W \in \mathbb{R}^{m \times d}\)
- Bias: \(b \in \mathbb{R}^{m}\)
- Output: \(z \in \mathbb{R}^{m}\)

---

## 2) Forward pass (compute the prediction)

For one linear layer:

\[
z = Wx + b
\]

Then apply an activation:

\[
a = \sigma(z)
\]

Common activations:
- ReLU: \(\sigma(z)=\max(0,z)\)
- Tanh: \(\sigma(z)=\tanh(z)\)

---

## 3) Loss (measure how wrong you are)

You need a scalar loss \(L\) so optimization can “push it down.”

Examples:
- Mean squared error (regression):
  \[
  L = \frac{1}{2}\|y - \hat{y}\|^2
  \]
- Cross-entropy (classification): depends on logits/probabilities

Backprop computes \(\nabla_\theta L\): derivatives of the loss with respect to all parameters \(\theta = \{W, b, ...\}\).

---

## 4) Backprop in one idea: the chain rule

If \(L\) depends on \(a\), and \(a\) depends on \(z\), and \(z\) depends on \(W\), then:

\[
\frac{\partial L}{\partial W} =
\frac{\partial L}{\partial a}\cdot
\frac{\partial a}{\partial z}\cdot
\frac{\partial z}{\partial W}
\]

Backprop is just an efficient way to apply this through many layers.

---

## 5) Gradients for a linear layer (the core derivation)

Assume we already have the “upstream gradient”:

\[
g_z = \frac{\partial L}{\partial z}
\]

For \(z = Wx + b\):

### Gradient w.r.t weights \(W\)

Each weight \(W_{ij}\) affects \(z_i\) via \(z_i = \sum_j W_{ij} x_j + b_i\).

So:

\[
\frac{\partial L}{\partial W_{ij}} = g_{z,i} \cdot x_j
\]

In matrix form:

\[
\frac{\partial L}{\partial W} = g_z \, x^T
\]

### Gradient w.r.t bias \(b\)

\[
\frac{\partial L}{\partial b} = g_z
\]

### Gradient w.r.t input \(x\)

\[
\frac{\partial L}{\partial x} = W^T g_z
\]

That last one is how gradients flow backward into earlier layers.

---

## 6) Gradients through an activation

If \(a = \sigma(z)\), then:

\[
g_z = g_a \odot \sigma'(z)
\]

Where:
- \(g_a = \frac{\partial L}{\partial a}\)
- \(\odot\) is elementwise multiply

Examples:

- ReLU: \(\sigma'(z)=1\) if \(z>0\) else 0
- Tanh: \(\sigma'(z)=1-\tanh(z)^2\)

---

## 7) Worked example (one hidden layer)

Network:

- hidden: \(z_1 = W_1 x + b_1\), \(a_1 = \text{ReLU}(z_1)\)
- output: \(z_2 = W_2 a_1 + b_2\), \(\hat{y} = z_2\) (simple regression)
- loss: \(L = \frac{1}{2}\|\hat{y}-y\|^2\)

Backprop steps:

1) \(g_{\hat{y}} = \hat{y} - y\)
2) output layer:
   - \(g_{z_2} = g_{\hat{y}}\)
   - \(\partial L/\partial W_2 = g_{z_2} a_1^T\)
   - \(\partial L/\partial b_2 = g_{z_2}\)
   - \(g_{a_1} = W_2^T g_{z_2}\)
3) ReLU:
   - \(g_{z_1} = g_{a_1} \odot \mathbf{1}[z_1>0]\)
4) hidden layer:
   - \(\partial L/\partial W_1 = g_{z_1} x^T\)
   - \(\partial L/\partial b_1 = g_{z_1}\)

That is backprop for an MLP.

---

## 8) Common mistakes

- **Shape confusion**
  - Always write down dimensions for \(W, x, z\).
- **Forgetting bias gradients**
  - Bias gradients are easy: they match the upstream gradient.
- **ReLU derivative at 0**
  - Use 0 or 1 consistently (frameworks typically choose 0).
- **Batching mistakes**
  - For batches, gradients sum/average across examples. Define which you use.

---

## 9) Checklist for implementing from scratch

- Forward pass produces intermediate tensors you’ll need in backprop (store \(z\) for activations).
- Backward functions return both parameter gradients and input gradients.
- You validate with a numerical gradient check on small networks.
- You test on a tiny dataset where you know the expected behavior.

