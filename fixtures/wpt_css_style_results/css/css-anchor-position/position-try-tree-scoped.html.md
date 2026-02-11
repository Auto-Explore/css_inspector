# css/css-anchor-position/position-try-tree-scoped.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-tree-scoped.html"
}
```

## style[0]

```css

  body { margin: 0; }

  @position-try --doc {
    left: 100px;
  }

  .abs {
    width: 100px;
    position: absolute;
    left: 999999px; /* force fallback */
  }

  #doc_pf_doc { position-try-fallbacks: --doc; }
  #doc_pf_outer { position-try-fallbacks: --outer; }
  #doc_pf_inner { position-try-fallbacks: --inner; }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

      @position-try --outer {
        left: 200px;
      }

      .abs {
        position: absolute;
        left: 999999px; /* force fallback */
      }

      #outer_pf_doc { position-try-fallbacks: --doc; }
      #outer_pf_outer { position-try-fallbacks: --outer; }
      #outer_pf_inner { position-try-fallbacks: --inner; }
    
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

          @position-try --inner {
            left: 300px;
          }

          .abs {
            position: absolute;
            left: 999999px; /* force fallback */
          }

          #inner_pf_doc { position-try-fallbacks: --doc; }
          #inner_pf_outer { position-try-fallbacks: --outer; }
          #inner_pf_inner { position-try-fallbacks: --inner; }
        
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[3]

```css

  #host_slotted_part {
     width: 100px;
  }
  @position-try --host-slot-part {
    left: 1px;
  }
  #host_slotted_part::part(part) {
    position-try-fallbacks: --host-slot-part;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[4]

```css

      @position-try --host-slot-part {
        left: 2px;
      }
      ::slotted(#slotted), :host {
        position: absolute;
        left: 999999px; /* force fallback */
        position-try-fallbacks: --host-slot-part;
      }
      #part {
        position: absolute;
        left: 999999px; /* force fallback */
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
