import { useEffect, useState } from "react";
import { Person, Camera } from "./types";

function App() {
  const [people, setPeople] = useState<Person[]>([]);

  useEffect(() => {
    fetch("http://baby:3000/people")
      .then((res) => res.json())
      .then((people: Person[]) => setPeople(people));
  }, []);

  const [cameras, setCameras] = useState<Camera[]>([]);

  useEffect(() => {
    fetch("http://baby:3000/cameras")
      .then((res) => res.json())
      .then((camera: Camera[]) => setCameras(camera));
  }, []);

  return (
    <div>
      {people.map((person) => (
        <p>
          {person.name} is {person.age} years old
        </p>
      ))}
    
      <p>Number of cameras: {cameras.length}</p>
      {cameras.map((camera) => (
        <p>
          {camera.model}
        </p>
      ))}
    </div>
  );
}

export default App;
