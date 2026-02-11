# css/selectors/featureless-003.html

```json
{
  "format_version": 3,
  "file": "css/selectors/featureless-003.html"
}
```

## style[0]

```css

        div {
          width: 100px;
          height: 25px;
        }
        .red { background-color: red; }
        .green { background-color: green; }

        :host .t1, .error {
          background-color: green;
        }
        div:host .t2,
        :host .t2 {
          background-color: green;
        }

        div:host .t3,
        *:host .t3 {
          background-color: red;
        }
        /* no t4 test needed, just leaving the element in
          to prevent careful rounding from being necessary. */
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
