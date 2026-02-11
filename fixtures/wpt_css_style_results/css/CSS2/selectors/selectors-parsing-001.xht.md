# css/CSS2/selectors/selectors-parsing-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/selectors/selectors-parsing-001.xht"
}
```

## style[0]

```css

   p { background: white; color: green; }
   p:invalidPseudoClass, p.test1 { color: yellow; background: red; }
   p::invalidPseudoElement, p.test2 { color: yellow; background: red; }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
