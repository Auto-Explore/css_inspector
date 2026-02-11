# css/CSS2/linebox/baseline-block-with-overflow-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/baseline-block-with-overflow-001-ref.html"
}
```

## style[0]

```css

section {
  margin-bottom: 1px;
}
.outer {
  display: inline-block;
  background: orange;
  padding-bottom: 20px;
}
.inner {
  width: 30px;
  height: 30px;
  overflow: hidden;
  background: blue;
}
.inline-block {
  display: inline-block;
}
.margin-bottom {
  margin-bottom: 30px;
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
