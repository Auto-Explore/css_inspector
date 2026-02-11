# css/CSS2/zindex/stack-floats-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/zindex/stack-floats-002.xht"
}
```

## style[0]

```css

   /* outer block */
   .container { width: 5em; height: 5em; border: solid; font: 20px/1 Ahem; background: red; display: block; }
   .inner { display: block; }

   /* top and bottom */
   .float { width: 5em; height: 3em; margin: 0 0 -5em 0; padding: 1em 0; background: red; float: left; }
   .inner .inline { color: lime; display: inline; }

   /* middle */
   .float .block { height: 3em; margin: 0; background: lime; display: block; }
   .inner .block { height: 3em; margin: 0; background: red; display: block; }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
