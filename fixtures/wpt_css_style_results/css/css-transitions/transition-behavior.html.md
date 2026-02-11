# css/css-transitions/transition-behavior.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/transition-behavior.html"
}
```

## style[0]

```css

.testcase {
  float: left;
  width: 100px;
  height: 100px;
  transition: all 1s;
}
#discrete {
  transition-behavior: allow-discrete;
}
.animated {
  float: right;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “transition-behavior”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
