# css/css-anchor-position/remove-position-try-rules-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/remove-position-try-rules-001.html"
}
```

## style[0]

```css

  @position-try --pf1 {
    left: auto;
    right: anchor(left);
    top: 200px;
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

  @position-try --pf2 {
    left: auto;
    right: anchor(left);
    top: 300px;
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

  #anchor {
    anchor-name: --a;
    margin-left: 100px;
    height: 100px;
    background: hotpink;
  }
  #box {
    position: absolute;
    position-anchor: --a;
    top: 100px;
    left: anchor(right);
    position-try-fallbacks: --pf1,--pf2;
    width: 50px;
    height: 50px;
    background: cyan;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
