# css/css-view-transitions/column-span-during-transition-doesnt-skip-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/column-span-during-transition-doesnt-skip-ref.html"
}
```

## style[0]

```css

html {
  background: pink;
}
#container {
  width: 500px;
  height: 500px;
  columns: 2;
}
#target {
  height: 200px;
  background: green;
  column-span: all;
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
