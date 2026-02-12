# css/css-cascade/revert-rule-basic.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-rule-basic.tentative.html"
}
```

## style[0]

```css

  #test1 {
    color: green;
  }
  #test1 {
    color: red;
    color: revert-rule;
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

  #test2 { /* Specificity: (0, 0, 1) */
    color: red;
  }
  #test2#test2#test2 { /* Specificity: (0, 0, 3) */
    color: red;
    color: revert-rule;
  }
  #test2#test2 { /* Specificity: (0, 0, 2) */
    color: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

  #test3 {
    color: red;
  }
  #test3 {
    color: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[3]

```css

  #test4 {              z-index: 1; }
  #test4 {              z-index: 2; }
  #test4 { z-index: -1; z-index: revert-rule; }
  #test4 { z-index: -1; z-index: revert-rule; }
  #test4 { z-index: -1; z-index: revert-rule; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
