# css/css-view-transitions/iframe-and-main-frame-transition-with-name-on-iframe-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/iframe-and-main-frame-transition-with-name-on-iframe-ref.html"
}
```

## style[0]

```css

iframe {
  position: fixed;
  top: 0;
  left: 0;
  width: 50vw;
  height: 50vh;
  border: 1px solid orange;
}

body {
  background: green;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

body {
  background: blue;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
