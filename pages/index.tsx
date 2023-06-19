import type { NextPage } from "next";
import Head from "next/head";
import { useState } from "react";
import { delegate, revoke } from "../api/pandapass";
import styles from "../styles/Home.module.css";
import {
  Input,
  Container,
  Box,
  Tabs,
  TabList,
  TabPanels,
  TabPanel,
  Tab,
  Button,
} from "@chakra-ui/react";

const Home: NextPage = () => {
  const [isLoading, setLoading] = useState(false);
  const [delegateAddress, setDelegateAddress] = useState("");
  const [tabIndex, setTabIndex] = useState(0);

  return (
    <div className={styles.container}>
      <Head>
        <title>Pandapass</title>
        <meta name="description" content="Pandapass" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <Container centerContent>
          <Tabs 
            onChange={(index) => setTabIndex(index)}
            isFitted
            w={400}

            variant={"enclosed"}
            align="center"
            size="lg"
          >
            <TabList>
              <Tab >Delegate</Tab>
              <Tab>Revoke</Tab>
            </TabList>

            <TabPanels>
              <TabPanel>
                <DelegateInput
                  delegateAddress={delegateAddress}
                  setDelegateAddress={setDelegateAddress}
                />
              </TabPanel>
              <TabPanel>
                <DelegateInput
                  delegateAddress={delegateAddress}
                  setDelegateAddress={setDelegateAddress}
                />
              </TabPanel>
              ð
            </TabPanels>
          </Tabs>
          <Button
            colorScheme="blue"
            onClick={async () => {
              try {
                setLoading(true);
                if (tabIndex == 0) await delegate(delegateAddress);
                else await revoke(delegateAddress);
                setLoading(false);
              } catch (e) {
                setLoading(false);
                throw e;
              }
            }}
          >
            {isLoading ? "Loading..." : "Confirm"}
          </Button>
        </Container>
      </main>
    </div>
  );
};
interface DelegateInputProps {
  delegateAddress: string;
  setDelegateAddress: (val: string) => void;
}
const DelegateInput = ({
  delegateAddress,
  setDelegateAddress,
}: DelegateInputProps) => {
  return (
    <Input
      value={delegateAddress}
      onChange={(e) => setDelegateAddress(e.target.value)}
      placeholder="Delegate Address"
    />
  );
};
export default Home;
