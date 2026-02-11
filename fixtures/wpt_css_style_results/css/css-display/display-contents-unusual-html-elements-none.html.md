# css/css-display/display-contents-unusual-html-elements-none.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-contents-unusual-html-elements-none.html"
}
```

## style[0]

```css

  /* Disable kerning because kerning may differ for different node tree. */
  html { font-kerning: none; font-feature-settings: "kern" off; }
  body { overflow: hidden }
  br, wbr, meter, progress, canvas, embed, object, audio, iframe, img, video,
  input, textarea, select {
    display: contents;
    border: 10px solid red;
    width: 200px; height: 200px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “font-feature-settings”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
