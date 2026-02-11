# css/CSS2/visuren/fixed-pos-stacking-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visuren/fixed-pos-stacking-001.xht"
}
```

## style[0]

```css

      div {width: 5em; height: 2em}

      /* absolute with 'z-index: auto' does not create a stacking context */
      #absolute {position: absolute; left: 1em; top: 3em; background: green}
      #absolute div {position: absolute}
      #absolute div div {position: absolute; z-index: -1; background: red}

      /* fixed with 'z-index: auto' *does* create a stacking context */
      #fixed {position: fixed; left: 10em; top: 3em; background: red}
      #fixed div {position: absolute}
      #fixed div div {position: absolute; z-index: -1; background: green}
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
