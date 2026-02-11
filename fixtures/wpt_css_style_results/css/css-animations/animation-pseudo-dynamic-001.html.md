# css/css-animations/animation-pseudo-dynamic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-pseudo-dynamic-001.html"
}
```

## style[0]

```css

@keyframes anim {
  from { background-color: red }
  to { background-color: red }
}
.test {
  display: flex;
}
.test::before {
  content: "";
  display: block;
  width: 100px;
  height: 100px;
  background-color: green;
}
.tweak::before {
  animation: anim 2s linear infinite;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
