# css/css-contain/contain-size-dynamic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-dynamic-001.html"
}
```

## style[0]

```css

  /* Selectors for contain */
  #none .wrapper {
      containt: none;
  }
  #size .wrapper {
      contain: size;
  }
  #none_to_size .wrapper {
      containt: none;
  }
  #size_to_none .wrapper {
      contain: size;
  }

  /* Selectors for testing sizing as empty */
  .wrapper {
      display: inline-block;
  }
  .box {
      display: inline-block;
      width: 50px;
      height: 5px;
      background: black;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
