# css/css-contain/content-visibility/animation-display-lock.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/animation-display-lock.html"
}
```

## style[0]

```css

  #container {
    content-visibility: visible;
    contain: style layout paint;
    contain-intrinsic-size: 0 100px;
  }
  @keyframes fade {
    from { opacity: 1; }
    to { opacity: 0;  }
  }
  #target {
    background: 'green';
    height: 100px;
    width: 100px;
  }
  .animate {
    animation: fade 10s linear 2 alternate;
  }
  .transition {
    transition: opacity 10s linear;
  }
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “contain”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
