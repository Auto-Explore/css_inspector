# css/css-variables/variable-css-wide-keywords.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-css-wide-keywords.html"
}
```

## style[0]

```css

  body {
    --is-initial: initial;

    --should-not-inherit: tomato;
    --should-inherit: lightgreen;

    --registered-inherits-should-not-inherit: tomato;
    --registered-should-not-inherit: tomato;
    --registered-inherits-should-inherit: lightgreen;
    --registered-should-inherit: lightgreen;
    --registered-should-revert: tomato;
    --registered-inherits-should-revert: tomato;
  }
  @property --registered-inherits-should-not-inherit {
    syntax: '<color>';
    initial-value: lightgreen;
    inherits: true;
  }
  @property --registered-should-not-inherit {
    syntax: '<color>';
    initial-value: lightgreen;
    inherits: false;
  }
  @property --registered-inherits-should-inherit {
    syntax: '<color>';
    initial-value: tomato;
    inherits: true;
  }
  @property --registered-should-inherit {
    syntax: '<color>';
    initial-value: tomato;
    inherits: false;
  }
  @property --registered-should-revert {
    syntax: '<color>';
    initial-value: orange;
    inherits: false;
  }
  @property --registered-inherits-should-revert {
    syntax: '<color>';
    initial-value: orange;
    inherits: true;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

  #regular-revert-layer {
    @layer {
      --should-not-inherit: lightgreen;
    }
    @layer {
      --should-not-inherit: revert-layer;
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

## style[2]

```css

  #registered-revert-layer {
    @layer {
      --registered-should-revert: lightgreen;
    }
    @layer {
      --registered-should-revert: revert-layer;
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

  #registered-revert-layer-inherits {
    @layer {
      --registered-inherits-should-revert: lightgreen;
    }
    @layer {
      --registered-inherits-should-revert: revert-layer;
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

  #regular-fallback-revert-layer {
    @layer {
      --should-not-inherit: lightgreen;
    }
    @layer {
      --should-not-inherit: var(--is-initial, revert-layer);
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

## style[5]

```css

  #registered-fallback-revert-layer {
    @layer {
      --registered-should-revert: lightgreen;
    }
    @layer {
      --registered-should-revert: var(--is-initial, revert-layer);
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

## style[6]

```css

  #registered-fallback-revert-layer-inherits {
    @layer {
      --registered-inherits-should-revert: lightgreen;
    }
    @layer {
      --registered-inherits-should-revert: var(--is-initial, revert-layer);
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
