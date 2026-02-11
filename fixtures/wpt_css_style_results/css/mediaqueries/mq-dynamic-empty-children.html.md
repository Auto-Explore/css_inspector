# css/mediaqueries/mq-dynamic-empty-children.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-dynamic-empty-children.html"
}
```

## style[0]

```css

      :root { background-color: red; }
      /* This one should never apply */
      @media (min-width: 1500px) {}
      /* This one should change and start matching */
      @media (max-width: 400px) {
        :root { background-color: lime; }
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
