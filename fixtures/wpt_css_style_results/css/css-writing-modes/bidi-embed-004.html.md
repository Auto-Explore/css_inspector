# css/css-writing-modes/bidi-embed-004.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/bidi-embed-004.html"
}
```

## style[0]

```css

.test span { direction: ltr; unicode-bidi: embed; }

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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
