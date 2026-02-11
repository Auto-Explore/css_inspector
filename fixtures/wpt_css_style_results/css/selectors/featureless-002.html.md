# css/selectors/featureless-002.html

```json
{
  "format_version": 3,
  "file": "css/selectors/featureless-002.html"
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

        /* Make sure a compound selector can match it */
        :host:host .t1 {
          background-color: green;
        }
        /* Make sure various compounds *can't* match it */
        div:host > .t2 {
          background-color: red;
        }
        :host.host > .t3 {
          background-color: red;
        }
        *:host > .t4 {
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
