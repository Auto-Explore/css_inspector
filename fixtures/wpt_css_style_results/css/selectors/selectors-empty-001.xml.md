# css/selectors/selectors-empty-001.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/selectors-empty-001.xml"
}
```

## style[0]

```css

   tests, tests * {
    display: block;
   }
   tests > * {
    border: solid thick red;
    height: 1em;
    padding: 4px;
    margin: 4px;
    color: white;
   }
   tests[type=positive] > :empty, tests[type=negative] > :not(:empty) {
    border: solid thick lime;
   }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
