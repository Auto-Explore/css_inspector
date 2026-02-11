# css/CSS2/normal-flow/block-in-inline-insert-010-nosplit-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/block-in-inline-insert-010-nosplit-ref.xht"
}
```

## style[0]

```css

   body &gt; span { border: 3px solid blue }
   body &gt; span &gt; span { border: 3px solid cyan }
   .notstart { border-left: none; }
   .notend { border-right: none; }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
