# css/css-text/hyphens/reference/hyphens-character-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text/hyphens/reference/hyphens-character-ref.html"
}
```

## style[0]

```css

        @font-face {
            font-family: roboto_hyphen;
            src: url(../resources/roboto_hyphenation_subset.ttf)
        }

        div {
        width: 30px;
            hyphens: auto;
            -webkit-hyphens: auto;
            font-family: roboto_hyphen, cursive;
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “-webkit-hyphens”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
