# css/css-values/tree-counting/calc-sibling-function-in-shadow-dom.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/calc-sibling-function-in-shadow-dom.html"
}
```

## style[0]

```css

    #host > * {
      orphans: sibling-index();
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

## style[1]

```css

         slot::slotted(*) {
           z-index: sibling-index();
           order: sibling-count();
         }
       
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
