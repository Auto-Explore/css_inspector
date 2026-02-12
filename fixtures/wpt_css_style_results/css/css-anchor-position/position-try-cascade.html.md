# css/css-anchor-position/position-try-cascade.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-cascade.html"
}
```

## style[0]

```css

  .cb {
    position: relative;
    width: 100px;
    height: 100px;
    background: lightpink;
    display: inline-block;
  }
  .abs {
    position: absolute;
    background: darkcyan;
    left: 0px;
    top: 0px;
    width: 150px; /* force fallback */
    height: 25px;
    position-try-fallbacks: --pf;
  }
  @position-try --pf {
    width: 50px;
    left: 50px;
    top: 50px;
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

  #abs_important {
    left: 10px !important;
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

  @keyframes anim {
    from { top: 20px; }
    to { top: 20px; }
  }
  #abs_animation {
    animation: anim 1000s steps(2, start) paused;
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

  #abs_transition {
    top: 50px;

    &.move {
      top: 10px !important;
      transition: top 1000s steps(2, start);
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[4]

```css

  #abs_revert {
    position-try-fallbacks: --pf-revert;
  }
  @layer author-layer {
    #abs_revert {
      top: 30px;
      left: 30px;
    }
  }
  #abs_revert {
    top: 20px;
    left: 20px;
    /* overflowing .cb to force --pf-revert to be applied */
    width: 200px;
    height: 200px;
  }
  @position-try --pf-revert {
    left: revert;
    top: revert-layer;
    width: 30px;
    height: 30px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
