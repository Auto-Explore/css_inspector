# css/CSS2/visufx/overflow-applies-to-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visufx/overflow-applies-to-001.xht"
}
```

## style[0]

```css

  #container {width: 20em; background: red; text-align: center;
    line-height: 1.2em}

  /* The visible overflow of grandchild1 hides the red container background */
  #child1 {width: 10em; overflow: visible}
  #grandchild1 {width: 20em; background: green}

  /* The red border on grandchild2 is clipped by child2 */
  #child2 {width: 20em; background: green; overflow: hidden}
  #grandchild2 {width: 20em; border-right: solid 1em red}

  /* Overflow (and width) don't apply to child3 and so grandchild3 is visible */
  #child3 {display: inline; width: 10em; overflow: hidden; background: red}
  #grandchild3 {width: 20em; height: 1.2em; background: red;
    vertical-align: bottom}

  /* Child4 establishes a formatting context and so can clip grandchild4 */
  #child4 {table-layout: fixed; width: 20em; border-collapse: collapse;
    background: green; overflow: hidden}
  #grandchild4 {width: 20em; background: green; border-right: solid 1em red;
    margin-right: -1em}
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
