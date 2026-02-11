# css/CSS2/generated-content/content-172.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/content-172.xht"
}
```

## style[0]

```css

   /* Setup */
   td { font: 1em/1em monospace; border: solid; }
   .normal div { white-space: normal; }
   .pre div { white-space: pre; }
   .nowrap div { white-space: nowrap; }
   :before, :after { white-space: inherit; /* this isn't a cascade test */ }

   /* Test */
   .test:before { content: 'XXX\A    XXX\A       '; }
   .test:after { content: 'XXX'; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
