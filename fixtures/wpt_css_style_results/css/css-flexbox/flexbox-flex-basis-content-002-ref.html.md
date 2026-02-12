# css/css-flexbox/flexbox-flex-basis-content-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-flex-basis-content-002-ref.html"
}
```

## style[0]

```css

  .container {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    border: 2px solid purple;
    padding: 2px;
    margin-right: 2em;
    width: 50px;
    height: 200px;
    float: left;
  }

  .container > * {
    flex-shrink: 0;
    min-height: 0;
    border: 2px solid teal;
  }

  .smallText { font: 10px Ahem; }
  .bigText   { font: 20px Ahem; }
  .spacerChild::before {
    content: '';
    display: block;
    background: brown;
    height: 10px;
    width: 10px;
  }
  .justPadding {
    /* Empty div with 5px padding on each side */
    padding: 5px;
    background: cyan;
  }
  canvas { background: fuchsia }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
