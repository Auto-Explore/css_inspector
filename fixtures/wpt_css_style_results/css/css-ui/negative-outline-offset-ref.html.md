# css/css-ui/negative-outline-offset-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/negative-outline-offset-ref.html"
}
```

## style[0]

```css

  div {
    border: 2px solid black;
    padding: 5px 0; /* No horizontal padding as outline-offset is not affected by it and span simulates it */
    font-size: 16px;
  }

  span {
    display: block;

    /* 3/6px offset for border */
    height: 2px;
    width: calc(100% - 20em + 6px);
    margin: -1px calc(10em - 3px); /* -1px vertical to remove height of span from div height */
    background: red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
