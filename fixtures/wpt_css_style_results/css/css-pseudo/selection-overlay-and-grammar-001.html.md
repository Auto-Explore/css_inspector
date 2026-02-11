# css/css-pseudo/selection-overlay-and-grammar-001.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/selection-overlay-and-grammar-001.html"
}
```

## style[0]

```css

  div
    {
      font-size: 60px;
      line-height: 90px;
    }

  div::selection
    {
      background-color: rgba(0%, 50%, 100%, 0.5);
      /*
      a very lite blue color
      according to
      https://www.colorhexa.com/7fbfff
      */
      color: yellow;
    }

  div::grammar-error
    {
      background-color: yellow;
      color: red;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
