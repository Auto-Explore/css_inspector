# css/css-contain/content-visibility/content-visibility-016.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-016.html"
}
```

## style[0]

```css

body {
  margin: 0;
  padding: 0;
}
#outer {
  width: 100px;
  height: 100px;
  background: lightblue;

  content-visibility: hidden;
  will-change: transform;
}
#inner {
  margin: 25px;
  width: 50px;
  height: 50px;
  background lightgreen;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
