import styles from "./Home.module.scss";
import Logo from "@/components/Logo";
import Navbar from "@/components/Navbar";

export default function Home() {
  return (
    <>
      <Navbar />
      <h1 className={styles.title}>
        <Logo /> Sintonia
      </h1>
    </>
  );
}
