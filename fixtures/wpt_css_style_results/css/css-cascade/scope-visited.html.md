# css/css-cascade/scope-visited.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-visited.html"
}
```

## style[0]

```css

      /* The visited background-color magically gets the alpha of the
         unvisited color, which by default is rgba(0, 0, 0, 0). Set alpha to
         255 so that visited colors also gets this alpha. */
      * { background-color: rgba(255, 255, 255, 255); }

      @scope (:visited) {
        :scope { background-color: coral; }
      }
      @scope (:link) {
        :scope { background-color: skyblue; } /* Does not match. */
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

      * { background-color: rgba(255, 255, 255, 255); }

      @scope (:link) {
        :scope { background-color: skyblue; }
      }
      @scope (:visited) {
        :scope { background-color: coral; } /* Does not match. */
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

      * { background-color: rgba(255, 255, 255, 255); }

      @scope (:visited) {
        :scope span { background-color: coral; }
      }
      @scope (:link) {
        :scope span { background-color: skyblue; } /* Does not match. */
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[3]

```css

      * { background-color: rgba(255, 255, 255, 255); }

      @scope (:link) {
        :scope span { background-color: skyblue; }
      }
      @scope (:visited) {
        :scope span { background-color: coral; } /* Does not match. */
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[4]

```css

      * { background-color: rgba(255, 255, 255, 255); }

      @scope (main) to (:visited) {
        /* Does not match, because #visited is not in scope. */
        :scope :visited { background-color: coral; }
      }
      @scope (main) {
        :scope :link { background-color: skyblue; } /* Also doesn't match. */
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[5]

```css

      * { background-color: rgba(255, 255, 255, 255); }

      @scope (main) to (:visited) {
        /* Does not match, because #unvisited does not match it. */
        :scope :visited { background-color: coral; }
      }
      @scope (main) {
        /* Should match, because the scope-end selector (:visited) does not
           match anything, hence we are in-scope. */
        :scope :link { background-color: skyblue; }
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[6]

```css

      * { background-color: rgba(255, 255, 255, 255); }

      @scope (main) to (:link) {
        /* Does not match, because #unvisited is not in scope. */
        :scope :link { background-color: skyblue; }
      }
      @scope (main) {
        :scope :visited { background-color: coral; } /* Also doesn't match. */
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[7]

```css

      * { background-color: rgba(255, 255, 255, 255); }

      @scope (main) to (:link) {
        /* Does not match, because #visited does not match it. */
        :scope :link { background-color: skyblue; }
      }
      @scope (main) {
        /* Should match, because the scope-end selector (:visited) does not
           match anything, hence we are in-scope. */
        :scope :visited { background-color: coral; }
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[8]

```css

      * { background-color: rgba(255, 255, 255, 255); }

      /* Should not match since visited-link matching stops applying
         once a link is seen. */
      @scope (:visited) {
        :scope > :visited { background-color: coral; }
      }
      @scope (:visited) {
        :visited > :scope { background-color: lightgrey; }
      }
      @scope(.with_class_inner) {
        :visited > :scope { color: yellow;  }
      }

      /* Should match */
      @scope(.with_class_outer) {
        :scope > :visited { color: green; }
      }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
