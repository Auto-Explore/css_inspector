# css/selectors/is-where-shadow.html

```json
{
  "format_version": 3,
  "file": "css/selectors/is-where-shadow.html"
}
```

## style[0]

```css

      * { color: blue; }
      :host(:is(.a, .b)) b { color: green; }
      :host-context(:is(.parent1, .parent2)) i { color: green; }
      ::slotted(:is(.e, .f)) { color: green; }

      /* The following should not match: */
      :host(:is(.z)) b { color: red; }
      :host(:is(.a + .b)) b { color: red; }
      :host-context(:is(.z)) i { color: red; }
      :host-context(:is(.parent1 .parent2)) i { color: red; }
      ::slotted(:is(.z)) { color: red; }
      ::slotted(:is(.a > .b)) { color: red; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
