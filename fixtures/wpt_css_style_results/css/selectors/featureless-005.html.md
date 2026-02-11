# css/selectors/featureless-005.html

```json
{
  "format_version": 3,
  "file": "css/selectors/featureless-005.html"
}
```

## style[0]

```css

        div {
          width: 100px;
          height: 50px;
        }
        .red { background-color: red; }
        .green { background-color: green; }

        :host:has(.t1) .t1 {
          background-color: green;
        }

        :has(.t2) .t2 {
          background-color: red;
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
