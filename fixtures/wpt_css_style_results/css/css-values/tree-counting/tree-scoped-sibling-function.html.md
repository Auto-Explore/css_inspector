# css/css-values/tree-counting/tree-scoped-sibling-function.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/tree-scoped-sibling-function.html"
}
```

## style[0]

```css

  #host1::part(--p) {
    z-index: sibling-index();
    /* Add 1 since widows does not accept 0 */
    widows: calc(1 + sibling-count());
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

        :host {
          z-index: sibling-index();
          widows: sibling-count();
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
