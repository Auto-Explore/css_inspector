# css/css-writing-modes/reference/bidi-isolate-override-009.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/reference/bidi-isolate-override-009.html"
}
```

## style[0]

```css

.test span { direction: ltr; unicode-bidi: isolate-override; }

 /* the following styles are not part of the test */
.test, .ref { font-size: 150%; border: 1px solid orange; margin: 10px; width: 10em; padding: 5px; clear: both; }
input { margin: 5px; }
@font-face {
    font-family: 'ezra_silregular';
    src: url('/fonts/sileot-webfont.woff') format('woff');
    font-weight: normal;
    font-style: normal;
    }
.test, .ref { font-family: ezra_silregular, serif; }
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
