import { createBrowserRouter } from "react-router-dom";
import Home from "./pages/Home";
import { About } from "./pages/About";
import { Contact } from "./pages/Contact";
import { Services } from "./pages/Services";

const sections = [Home, About, Contact, Services];

const router = createBrowserRouter([
  { path: "/", element: <Home /> },
  { path: "about", element: <About /> },
  { path: "contact", element: <Contact /> },
  { path: "services", element: <Services /> },
  { path: "*", element: <Home /> },
]);

export { router, sections };
