# css/selectors/featureless-001.html

```json
{
  "format_version": 3,
  "file": "css/selectors/featureless-001.html"
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
        :host div.red {
          /* Make sure :host matches the host element... */
          background-color: green;
        }
        div > div.green {
          /* And make sure *other* selectors *don't* match it. */
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
