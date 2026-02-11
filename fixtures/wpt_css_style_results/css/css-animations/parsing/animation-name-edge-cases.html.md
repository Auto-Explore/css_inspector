# css/css-animations/parsing/animation-name-edge-cases.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/parsing/animation-name-edge-cases.html"
}
```

## style[0]

```css

  @keyframes "" {
    to { background: red }
  }
  @keyframes default {
    to { background: red }
  }
  @keyframes "default" {
    to { background: green }
  }
  @keyframes "string" {
    to { background: green }
  }
  @keyframes "none" {
    to { background: green }
  }
  @keyframes "inherit" {
    to { background: green }
  }
  @keyframes "initial" {
    to { background: green }
  }
  @keyframes "revert" {
    to { background: green }
  }
  @keyframes "revert-layer" {
    to { background: green }
  }
  @keyframes "revert-rule" {
    to { background: green }
  }
  @keyframes "unset" {
    to { background: green }
  }

  .test {
    width: 300px;
    height: 25px;
    margin-top: 10px;
    background-color: red;
    animation: 0s both;
  }

  .test-invalid {
    background-color: green;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
