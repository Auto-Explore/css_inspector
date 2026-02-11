# css/cssom/getComputedStyle-insets-absolute-crash.html

```json
{
  "format_version": 3,
  "file": "css/cssom/getComputedStyle-insets-absolute-crash.html"
}
```

## style[0]

```css

.container {
  position: relative;
  width: 100px;
  height: 100px;
  background: lime;
}

.oof {
  position: absolute;
  width: 30px;
  height: 30px;
  top: 0;
  left: 0;
  overflow: hidden;
  background: hotpink;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
