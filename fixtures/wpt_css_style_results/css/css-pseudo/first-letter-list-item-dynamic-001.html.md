# css/css-pseudo/first-letter-list-item-dynamic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/first-letter-list-item-dynamic-001.html"
}
```

## style[0]

```css

  #outer {
    display: list-item;
    overflow: auto; /* Triggers a crash in Blink see link to issue 977044 */
  }
  #outer::first-letter {
    float: left;
    color: green;
  }
  #fixed { position: fixed; }
  #inner { float: left; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
