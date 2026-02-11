# css/css-flexbox/flexbox-align-items-center-nested-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-items-center-nested-001.html"
}
```

## style[0]

```css

html, body { margin: 0; padding: 0; }
body {
  height: 400px;
  position: relative;
}

.container-0 {
  display: flex;
  position: absolute;
  height: 100%;
  flex-direction: column;
}

.container-1 {
  flex: 1 0 auto;
  display: flex;
  align-items: center;
}

.container-2 {
  height: 100%;
  display: flex;
  align-items: center;
}

.content {
  width: 200px;
  height: 200px;
  background: blue;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
