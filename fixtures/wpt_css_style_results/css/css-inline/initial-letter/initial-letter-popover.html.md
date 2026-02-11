# css/css-inline/initial-letter/initial-letter-popover.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/initial-letter-popover.html"
}
```

## style[0]

```css

  #block {
    background-color: green;
    color: green;
    width: 100px;
    height: 100px;
    text-indent: 1em; /* Avoid 'X' sticking out of the square */
  }
  #block::first-letter {
    initial-letter: 1;
  }
  #pop, #but { visibility: hidden; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
