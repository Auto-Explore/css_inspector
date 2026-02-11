# css/css-cascade/scope-name-defining-rules.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-name-defining-rules.html"
}
```

## style[0]

```css

    @scope (#inner) {
      @keyframes --my-anim {
        from { background-color: rgb(0, 0, 255); }
        to { background-color: rgb(0, 0, 255); }
      }
    }
    @scope (#outer) {
      @keyframes --my-anim {
        from { background-color: rgb(0, 128, 0); }
        to { background-color: rgb(0, 128, 0); }
      }
    }
    #animating {
      animation: --my-anim 1000s linear;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

    @scope (#nomatch) {
      @keyframes --my-anim {
        from { background-color: rgb(0, 128, 0); }
        to { background-color: rgb(0, 128, 0); }
      }
    }
    #animating {
      animation: --my-anim 1000s linear;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

    @scope (#inner) {
      @property --my-prop {
        syntax: "<length>";
        initial-value: 1px;
        inherits: false;
      }
    }
    @scope (#outer) {
      @property --my-prop {
        syntax: "<length>";
        initial-value: 2px;
        inherits: false;
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

## style[3]

```css

    @scope (#nomatch) {
      @property --my-prop {
        syntax: "<length>";
        initial-value: 2px;
        inherits: false;
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
