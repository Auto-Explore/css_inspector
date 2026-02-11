# css/css-view-transitions/root-style-change-during-animation-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/root-style-change-during-animation-ref.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  contain: paint;
  background: green;
}

body {
  background: grey;
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
