# css/selectors/not-links.html

```json
{
  "format_version": 3,
  "file": "css/selectors/not-links.html"
}
```

## style[0]

```css

  * {
    /* Browsers typically ignore the :visited alpha and use the unvisited
       alpha instead, which by default is 0, in which case a failure would
       not be detected. */
    background-color: white;
  }
  div *:not(:link):not(:visited) {
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
