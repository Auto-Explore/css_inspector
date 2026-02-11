# css/css-transforms/transform-root-bg-003.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-root-bg-003.html"
}
```

## style[0]

```css

      html {
        background: url(support/transform-triangle-left.svg);
        transform: scale(0.5);
        /**
         * The transform-origin here has to fall between two triangles, i.e.,
         * at a multiple of 100px.  Otherwise after the transform, the shrunken
         * images won't line up with the left edge of the body, and it won't
         * match the ref (since the background here is positioned at the left).
         * We deliberately make it an odd multiple of the number of images so
         * it catches an IE bug; it shouldn't matter per spec
         */
        transform-origin: 300px 0;
      }
      body {
        margin: 0;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
