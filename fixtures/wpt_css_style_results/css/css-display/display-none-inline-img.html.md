# css/css-display/display-none-inline-img.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-none-inline-img.html"
}
```

## style[0]

```css

.carousel {
  position: relative;
  overflow: hidden;
  height: 300px;
}

.scroller {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  overflow: auto;
}

.slides {
  width: 200%;
  display: flex;
}

.slide {
  flex: 0 0 50%;
  contain:  paint;
}

img {
  width: 300px;
  height: 250px;
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
