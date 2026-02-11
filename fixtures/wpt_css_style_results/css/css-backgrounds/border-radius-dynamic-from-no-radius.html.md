# css/css-backgrounds/border-radius-dynamic-from-no-radius.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-radius-dynamic-from-no-radius.html"
}
```

## style[0]

```css

  #outer {
    width: 100px;
    height: 100px;
  }
  #inner {
    width: 100%;
    height: 100%;
    background: green;
    /* The key is that this starts off computing to zero */
    border-radius: calc(100% - 1px);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
