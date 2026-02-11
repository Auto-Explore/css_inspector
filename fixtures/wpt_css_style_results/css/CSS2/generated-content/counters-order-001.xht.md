# css/CSS2/generated-content/counters-order-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/counters-order-001.xht"
}
```

## style[0]

```css


  ul { display: block; margin: 0; padding: 0; counter-reset: c; }
  li { counter-increment: c; }
  li, div { display: block; margin: 0; padding: 0;
            width: 3em; border: thin solid; }
  li:before, div:before { content: counter(c); }

  #four { border: none; }
  #four:before { content: none; }

  #two { float: left; }
  #three { position: relative; }
  #four { position: relative; }
  #four div { position: absolute; left: 8em; }
  #six { position: absolute; top: 5em; left: 12em; }
  #eight { position: fixed; top: 8em; left: 4em; }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
