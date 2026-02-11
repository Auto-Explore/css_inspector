# css/css-pseudo/active-selection-063.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/active-selection-063.html"
}
```

## style[0]

```css

  div
    {
      background-color: red;
      color: red;
      float: left; /* or display: inline-block or position: absolute or width: 4em */
      font-family: Ahem;
      font-size: 25px;
      line-height: 1;
      /* -moz-tab-size: 4; */
      /*
      Implement 'tab-size' (dropping the -moz- prefix)
      https://bugzilla.mozilla.org/show_bug.cgi?id=737785
      */
      tab-size: 4;
      white-space: pre;
    }

  div::selection
    {
      background-color: green;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
