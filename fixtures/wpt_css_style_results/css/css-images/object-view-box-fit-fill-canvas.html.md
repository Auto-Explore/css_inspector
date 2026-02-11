# css/css-images/object-view-box-fit-fill-canvas.html

```json
{
  "format_version": 3,
  "file": "css/css-images/object-view-box-fit-fill-canvas.html"
}
```

## style[0]

```css

/* The test uses clip-path to avoid comparing edges with minor pixel differences
   due to differences in scaling on highdpi devices */

.view_box_subset {
  object-view-box: inset(50px 0px 0px 0px);
  object-fit: fill;
  margin: 5px;
  clip-path: inset(1px 0px 0px 0px);
}

.view_box_subset_with_position {
  object-view-box: inset(50px 0px 0px 0px);
  object-fit: fill;
  margin: 5px;
  object-position: 10px 10px;
  background-color: grey;
}

.view_box_subset_with_scaling {
  object-view-box: inset(50px 0px 0px 0px);
  object-fit: fill;
  margin: 5px;
  width: 50px;
  height: 100px;

  /* The top row of pixels can have minor differences due to difference in order
     of clipping and scaling operations */
  clip-path: inset(1px 0px 0px 0px);
}

.view_box_superset {
  object-view-box: inset(0px -50px 0px 0px);
  object-fit: fill;
  margin: 5px;
  background-color: grey;
}

.view_box_superset_with_position {
  object-view-box: inset(0px -50px 0px 0px);
  object-fit: fill;
  margin: 5px;
  background-color: grey;
  object-position: 10px 10px;
}

.view_box_superset_with_scaling {
  object-view-box: inset(0px -50px 0px 0px);
  object-fit: fill;
  margin: 5px;
  background-color: grey;
  width: 50px;
  height: 50px;
}

.view_box_intersection {
  object-view-box: inset(-50px 25px 50px 0px);
  object-fit: fill;
  margin: 5px;
  background-color: grey;
  clip-path: inset(0px 0px 1px 0px);
}

.view_box_no_intersection {
  object-view-box: inset(-50px 25px 100px 0px);
  object-fit: fill;
  margin: 5px;
  background-color: grey;
}
```

```json
{
  "errors": 16,
  "messages": [
    {
      "message": "Unknown property “object-view-box”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “object-view-box”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “object-view-box”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “object-view-box”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “object-view-box”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “object-view-box”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “object-view-box”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “object-view-box”.",
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
